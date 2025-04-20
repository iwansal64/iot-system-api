import { FastifyReply, FastifyRequest } from "fastify";
import { prisma } from "../database/database";
import { ERROR_DEVICE_NOTFOUND, ERROR_REQUEST_BODY_NOTCOMPLETE, ERROR_UNKNOWN_ERROR } from "../error/errors";
import { PrismaClientKnownRequestError } from "@prisma/client/runtime/library";

interface SetOnlineBody {
    device_token?: string
}

interface SetOfflineBody {
    device_token?: string
}

export async function set_online(req: FastifyRequest<{ Body: SetOnlineBody }>, res: FastifyReply) {
    // Get the required body
    const { device_token: target_device_token } = req.body;
    if(!target_device_token) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }
    

    // Activate the device
    try {
        await prisma.device.update({
            where: {
                token: target_device_token
            },
            data: {
                status: 1,
                last_online: new Date()
            }
        });
    }
    catch(error) {
        if(error instanceof PrismaClientKnownRequestError) {
            return res.code(404).send(ERROR_DEVICE_NOTFOUND);
        }

        console.error(`There's an error when trying to activate the device. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }


    return {
        message: "Successfully set the device status to online",
        success: true
    }
}

export async function set_offline(req: FastifyRequest<{ Body: SetOfflineBody }>, res: FastifyReply) {
    // Get the required body
    const { device_token: target_device_key } = req.body;
    if(!target_device_key) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }
    

    // Activate the device
    try {
        await prisma.device.update({
            where: {
                id: target_device_key
            },
            data: {
                status: 0,
            }
        });
    }
    catch(error) {
        if(error instanceof PrismaClientKnownRequestError) {
            return res.code(404).send(ERROR_DEVICE_NOTFOUND);
        }
        
        console.error(`There's an error when trying to activate the device. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }


    return {
        message: "Successfully set the device status to offline",
        success: true
    }
}