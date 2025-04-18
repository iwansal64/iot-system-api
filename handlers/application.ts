import { FastifyReply, FastifyRequest } from "fastify";
import { ERROR_REGISTRATIONTOKEN_ISWRONG, ERROR_REGISTRATIONTOKEN_NOTFOUND, ERROR_REQUEST_BODY_NOTCOMPLETE, ERROR_UNKNOWN_ERROR } from "../error/errors";
import { prisma } from "../database/database";
import { generate_verification_token } from "../utilities";
import nodemailer from "nodemailer";
import jwt from "jsonwebtoken";

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
            },
            update: {}
        })
    }
    catch(error) {
        console.error(`There's an error when trying to upsert user`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }

    return {
        message: "Succesfully logged in with email",
        success: true,
        data: {
            token: jwt.sign(
                {
                    email: verification_key_data.email
                }, 
                process.env.USER_JWT_TOKEN_SECRET!,
                { 
                    expiresIn: "1h" 
                }
            )
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
        from: "iwantest64@gmail.com",
        to: user_given_email,
        subject: "IoT App Email Confirmation",
        html: `<div style="font-size: 18px; letter-spacing: 1px;">
            <p>Hello, <i>${user_given_email.split("@")[0]}</i>! I'm here to ask for an email confirmation to regiter your account into IoT App! Copy token below and paste it into the next registration process. If you sure this is a mistake you can ignore this message, thank you! I hope you have a good day today!</p>
            <br>
            <p>Token: <b>[${generated_token}]</b></p>
            <div style=\"display: flex;\">
                ${Array.from(generated_token).map(key => "<div style=\"font-size: 36px; margin-left: 10px; font-weight: bold; border: 3px solid black; border-radius: 10px; padding: 10px; width: 50px; height: 50px; display: flex; justify-content: center; align-items: center; text-align: center\"><div>"+key+"</div></div>").join("")}
            </div>
        </div>`,
        date: new Date("2025-01-01T10:00:12")
    }).catch((error) => {
        console.error(`There's an error when trying to send a message to an email. Error: ${error}`);
    });


    return {
        message: "Successfully sign up. need email confirmation.",
        success: true,
        data: {
            id: verification_key_data.id
        }
    }
}
