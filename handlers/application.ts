import { FastifyReply, FastifyRequest } from "fastify";
import { ERROR_CATEGORY_NOTFOUND, ERROR_CONTROLLABLE_NOTFOUND, ERROR_DATABASE_DUPLICATE, ERROR_REGISTRATIONTOKEN_ISWRONG, ERROR_REGISTRATIONTOKEN_NOTFOUND, ERROR_REQUEST_BODY_NOTCOMPLETE, ERROR_UNKNOWN_ERROR, ERROR_USER_NOTFOUND, ERROR_USER_UNAUTHORIZED } from "../error/errors";
import { prisma } from "../database/database";
import { generate_device_pass, generate_mqtt_user_and_pass, generate_topic_name, generate_verification_token } from "../utilities";
import nodemailer from "nodemailer";
import { generate_device_key } from "../utilities";
import { user_authentication } from "../middlewares/security";
import { Controllable, Device, Prisma } from "../generated/prisma";
import { is_duplicate } from "../middlewares/db_process";

const email_transporter = nodemailer.createTransport({
    service: "gmail",
    auth: {
        user: process.env.EMAIL_USER!,
        pass: process.env.EMAIL_PASS!
    }
});


interface LoginBody {
    email?: string
}


interface VerifyBody {
    id?: string,
    token?: string
}

interface RequestKeyBody {
    device_name?: string
}

interface CreateControllableBody {
    device_id?: string,
    name?: string
    category?: string,
}

interface GetControllableBody {
    controllable_name?: string
    controllable_device_id?: string
}

export async function verify_user(req: FastifyRequest<{ Body: VerifyBody }>, res: FastifyReply) {
    // Get the required body data
    const { id: user_given_id, token: user_given_token } = req.body;
    if(!user_given_id || !user_given_token) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }

    // Get the verification key data
    const verification_key_data = await prisma.userVerificationKeys.findUnique({
        where: {
            id: user_given_id,
        }
    });

    if(!verification_key_data) {
        return res.code(401).send(ERROR_REGISTRATIONTOKEN_NOTFOUND);
    }


    // Verify the token
    if(user_given_token != verification_key_data.token) {
        return res.code(500).send(ERROR_REGISTRATIONTOKEN_ISWRONG);
    }

    // Create or Update a new user
    try {
        await prisma.user.upsert({
            where: {
                email: verification_key_data.email
            },
            create: {
                email: verification_key_data.email,
                username: verification_key_data.email.split("@")[0],
                mqtt_pass: generate_mqtt_user_and_pass(),
                mqtt_user: generate_mqtt_user_and_pass()
            },
            update: {}
        })
    }
    catch(error) {
        console.error(`There's an error when trying to upsert user`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }

    const token = await res.jwtSign({ email: verification_key_data.email });
    res.setCookie("user_token", token);

    return {
        message: "Succesfully logged in with email",
        success: true,
        data: {
            token: token
        }
    }
}

export async function log_in_user(req: FastifyRequest<{ Body: LoginBody }>, res: FastifyReply) {
    // Get the required body data
    const { email: user_given_email } = req.body;

    if(!user_given_email) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }

    // Create a verification key
    const generated_token = generate_verification_token();
    const verification_key_data = await prisma.userVerificationKeys.create({
        data: {
            email: user_given_email,
            token: generated_token
        }
    });
    

    // Send verification email      
    email_transporter.sendMail({
        from: `"IoT Connect" <${process.env.EMAIL_USER!}>`,
        to: user_given_email,
        subject: "IoT App Email Confirmation",
        html: `<div style="font-size: 18px; letter-spacing: 1px;">
            <p>Hello, <i>${user_given_email.split("@")[0]}</i>! I'm here to ask for an email confirmation to regiter your account into IoT App! Copy token below and paste it into the next registration process. If you sure this is a mistake you can ignore this message, thank you! I hope you have a good day today!</p>
            <br>
            <p>Token: <b>[${generated_token}]</b></p>
            <div style=\"display: flex; gap: 4px\">
                ${Array.from(generated_token).map(key => "<div style=\"font-size: 36px; margin-left: 10px; font-weight: bold; border: 3px solid black; border-radius: 10px; padding: 10px; width: 50px; height: 50px; display: flex; justify-content: center; align-items: center; text-align: center\"><div>"+key+"</div></div>").join("")}
            </div>
        </div>`,
        date: new Date("2025-01-01T10:00:12")
    }).catch((error) => {
        console.error(`There's an error when trying to send a message to an email. Error: ${error}`);
    });


    return {
        success: true,
        data: {
            id: verification_key_data.id
        }
    }
}

export async function log_out_user(req: FastifyRequest, res: FastifyReply) {
    res.cookies.user_token = undefined;
    return {
        success: true
    };
}


export async function create_device(req: FastifyRequest<{ Body: RequestKeyBody }>, res: FastifyReply) {
    // Check user authentication
    if(!(await user_authentication(req, res))) {
        return res.code(401).send(ERROR_USER_UNAUTHORIZED);
    }


    // Get the required data
    const { device_name } = req.body;

    if(!device_name) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }

    // Save the device key to database
    const generated_device_key = generate_device_key();
    const generated_device_pass = generate_device_pass();
    let device_data: Device | null;
    try {
        device_data = await prisma.device.create({
            data: {
                name: device_name,
                device_key: generated_device_key,
                device_pass: generated_device_pass,
                user: {
                    connect: {
                        email: req.user.toString()
                    }
                }
            }
        });
    }
    catch(error) {
        console.error(`There's an error when trying to create device key. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR)
    }

    
    return {
        success: true,
        data: {
            device_key: device_data.device_key,
            device_pass: device_data.device_pass,
            device_id: device_data.id
        }
    }
}

export async function create_controllable(req: FastifyRequest<{ Body: CreateControllableBody }>, res: FastifyReply) {
    // Check user authentication
    if(!(await user_authentication(req, res))) {
        return res.code(401).send(ERROR_USER_UNAUTHORIZED);
    }

    
    // Get the required data
    const { category, name, device_id } = req.body;
    if(!category || !name || !device_id) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }


    // Get category data to verify category name
    const category_data = await prisma.controllableCategory.findUnique({
        where: {
            category_name: category
        }
    });

    if(!category_data) {
        return res.code(404).send(ERROR_CATEGORY_NOTFOUND);
    }


    // Create controllable data
    let controllable_data: Controllable | null;
    try {
        controllable_data = await prisma.controllable.create({
            data: {
                name: name,
                controller_device: {
                    connect: {
                        id: device_id
                    }
                },
                controllable_category: {
                    connect: {
                        category_name: category
                    }
                },
                topic_name: generate_topic_name()
            }
        });
    }
    catch(error) {
        if(is_duplicate(error)) {
            return res.code(400).send(ERROR_DATABASE_DUPLICATE);
        }
        
        console.error(`There's an error when trying to create controllable data. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }



    return {
        success: true,
        data: controllable_data
    }
}

export async function get_controllable_data(req: FastifyRequest<{ Body: GetControllableBody }>, res: FastifyReply) {
    // Check user authentication
    if(!(await user_authentication(req, res))) {
        return res.code(401).send(ERROR_USER_UNAUTHORIZED);
    }

    
    // Get the required data
    const { controllable_name, controllable_device_id } = req.body;
    if(!controllable_name || !controllable_device_id) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }


    // Get controllable data
    const controllable_data = await prisma.controllable.findUnique({
        where: {
            name_device_id: {
                device_id: controllable_device_id,
                name: controllable_name
            }
        }
    });

    if(!controllable_data) {
        return res.code(404).send(ERROR_CONTROLLABLE_NOTFOUND);
    }


    return {
        success: true,
        data: controllable_data
    }
}

export async function get_user_data(req: FastifyRequest, res: FastifyReply) {
    // Check user authentication
    if(!(await user_authentication(req, res))) {
        return res.code(401).send(ERROR_USER_UNAUTHORIZED);
    }

    const user_email = req.user;
    if(typeof user_email !== "string") {
        return res.code(401).send(ERROR_USER_UNAUTHORIZED);
    }


    // Get the user data
    const user_data = await prisma.user.findUnique({
        where: {
            email: user_email
        }
    });

    if(!user_data) {
        return res.code(404).send(ERROR_USER_NOTFOUND);
    }

    return {
        success: true,
        data: user_data
    }
}