import { FastifyReply, FastifyRequest } from "fastify";
import { ERROR_REGISTRATIONTOKEN_ISWRONG, ERROR_REGISTRATIONTOKEN_NOTFOUND, ERROR_REQUEST_BODY_NOTCOMPLETE, ERROR_UNKNOWN_ERROR } from "../error/errors";
import { prisma } from "../database/database";
import { generate_verification_token } from "../utilities";
import nodemailer from "nodemailer";

const email_transporter = nodemailer.createTransport({
    service: "gmail",
    auth: {
        user: process.env.EMAIL_USER,
        pass: process.env.EMAIL_PASS
    }
});


interface SignUpBody {
    email: string
}

interface LogInBody {
    email_or_username: string,
    password: string
}

interface VerifyBody {
    id: string,
    token: string
}

interface PostVerifyBody {
    
}

export async function verify(req: FastifyRequest<{ Body: VerifyBody }>, res: FastifyReply) {
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

    return {
        message: "Email successfully registered",
        success: true,
        data: {

        }
    }
}

export async function sign_up(req: FastifyRequest<{ Body: SignUpBody }>, res: FastifyReply) {
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
    try {
        await email_transporter.sendMail({
            from: "iwantest64@gmail.com",
            to: user_given_email,
            subject: "IoT App Email Confirmation",
            text: `Hello, ${user_given_email.split("@")[0]}! I'm here to ask for an email confirmation to regiter your account into IoT App! Copy token below and paste it into the next registration process. If you sure this is a mistake you can ignore this message, thank you! I hope you have a good day today!\n\nToken: [${generated_token}]`,
            html: `<div style="font-size: 36px; letter-spacing: 2px;">
                ${Array.from(generated_token).map(key => "<span>"+key+"</span>")}
            </div>`,
            date: new Date("2025-01-01T10:00:12")
        });
    }
    catch(error) {
        console.error(`There's an error when trying to send a message to an email. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }


    return {
        message: "Successfully sign up. need email confirmation.",
        success: true,
        data: {
            id: verification_key_data.id
        }
    }
}

export async function log_in(req: FastifyRequest<{ Body: LogInBody }>, res: FastifyReply) {
    
}