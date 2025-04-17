import { FastifyReply, FastifyRequest } from "fastify";
import { prisma } from "../database/database";
import { ERROR_DEVICE_NOTFOUND, ERROR_REQUEST_BODY_NOTCOMPLETE, ERROR_UNKNOWN_ERROR } from '../error/errors';
import { Device } from "../generated/prisma";

interface InitializeDeviceBody {
    device_key?: string
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
    const new_device_key_data = await prisma.deviceKey.create({
        data: {
            associated_device: {
                create: {
                    name: device_name,
                }
            }
        }
    });

    
    return {
        key: new_device_key_data.device_key
    };
}

export async function initialize_device(req: FastifyRequest<{ Body: InitializeDeviceBody }>, res: FastifyReply) {
    // Get the device key
    const { device_key: target_device_key } = req.body;

    if(!target_device_key) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }

    // Verify the device key
    let device_data: Device | null;
    try {
        device_data = await prisma.device.findUnique({
            where: {
                id: target_device_key,
                device_key: {
                    some: {
                        device_key: target_device_key
                    }
                }
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
        console.error(`There's an error when trying to activate the device. Error: ${error}`);
        return res.code(500).send(ERROR_REQUEST_BODY_NOTCOMPLETE);
    }

    // Delete the device key
    try {
        await prisma.deviceKey.deleteMany({
            where: {
                device_key: device_data.id
            }
        })
    }
    catch(error) {
        console.error(`There's an error when trying to delete the device key. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR);
    }

    return {
        message: "Successfully initialize device!",
        success: true,
        data: device_data
    }
}