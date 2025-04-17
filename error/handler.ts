import { FastifyError, FastifyReply, FastifyRequest } from "fastify";
import { ERROR_UNKNOWN_ERROR } from "./errors";

export async function error_handler(error: FastifyError, req: FastifyRequest, res: FastifyReply) {
    console.error();
    console.error();
    console.error(`===============================`);
    console.error(`There's an unknown error. Error: ${error}`);
    console.error(`===============================`);
    console.error();
    console.error();
    return res.code(500).send(ERROR_UNKNOWN_ERROR);
}