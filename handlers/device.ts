import { FastifyReply, FastifyRequest } from "fastify";
import { prisma } from "../database/database";
import { ERROR_DEVICE_NOTFOUND, ERROR_REQUEST_BODY_NOTCOMPLETE, ERROR_UNKNOWN_ERROR } from '../error/errors';
import { Device } from "../generated/prisma";
import { generate_device_token } from "../utilities";

interface InitializeDeviceBody {
    device_token?: string
}

interface RequestKeyBody {
    device_name?: string
}

export async function request_key(req: FastifyRequest<{ Body: RequestKeyBody }>, res: FastifyReply) {
    // Get the required data
    const { device_name } = req.body;

    if(!device_name) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }

    // Save the device key to database
    const generated_device_token = generate_device_token();
    try {
        await prisma.device.create({
            data: {
                name: device_name,
                token: generated_device_token
            }
        });
    }
    catch(error) {
        console.error(`There's an error when trying to create device key`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR)
    }

    
    return {
        token: generated_device_token
    };
}

export async function initialize_device(req: FastifyRequest<{ Body: InitializeDeviceBody }>, res: FastifyReply) {
    // Get the device key
    const { device_token: target_device_token } = req.body;

    if(!target_device_token) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }

    // Verify the device key
    let device_data: Device | null;
    try {
        device_data = await prisma.device.findUnique({
            where: {
                token: target_device_token,
            }
        });

        if(!device_data) {
            return res.code(404).send(ERROR_DEVICE_NOTFOUND);
        }
    }
    catch(error) {
        console.error(`There's an error when trying to get the device data. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }

    // Update the device status
    try {
        await prisma.device.update({
            where: {
                id: device_data.id
            },
            data: {
                status: 0
            }
        });
    }
    catch(error) {
        console.error(`There's an error when trying to update the device status. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }

    return {
        message: "Successfully initialize device!",
        success: true,
        data: device_data
    }
}