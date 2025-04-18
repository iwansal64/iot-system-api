import { FastifyReply, FastifyRequest } from "fastify";
import { ERROR_APIKEY_INVALID } from "../error/errors";

export async function apikey_validator(req: FastifyRequest, res: FastifyReply) {
    if(!req.headers.authorization || req.headers.authorization != process.env.API_KEY) {
        return res.code(401).send(ERROR_APIKEY_INVALID);
    }
}

export async function server_logger(req: FastifyRequest, res: FastifyReply) {
    console.log(`[LOGGER] ip=[${req.ip}]. body=[${JSON.stringify(req.body)}]`);
}