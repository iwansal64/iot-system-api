import { FastifyReply, FastifyRequest } from "fastify";
import { ERROR_APIKEY_INVALID } from "../error/errors";
import jwt from 'jsonwebtoken';

export async function apikey_validator(req: FastifyRequest, res: FastifyReply) {
    if(!req.headers.authorization || req.headers.authorization != process.env.API_KEY) {
        return res.code(401).send(ERROR_APIKEY_INVALID);
    }
}

export async function server_logger(req: FastifyRequest, res: FastifyReply) {
    console.log(`[LOGGER] ip=[${req.ip}]. url=[${req.url}]`);
}

export async function user_authentication(req: FastifyRequest, res: FastifyReply) {
    try {
        req.user = jwt.verify(req.cookies["user_token"]!, process.env.JWT_TOKEN!)!["email"];
        console.log(`User Email: ${req.user}`);


        if(!req.user) {
            return false
        }
    }
    catch(error) {
        return false;
    }

    return true;
}