import { FastifyReply, FastifyRequest } from "fastify";
import { prisma } from "../database/database";
import { ERROR_CONTROLLABLE_NOTFOUND, ERROR_DEVICE_NOTFOUND, ERROR_DEVICE_UNAUTHORIZED, ERROR_DEVICE_WRONGPASS, ERROR_REQUEST_BODY_NOTCOMPLETE, ERROR_UNKNOWN_ERROR, ERROR_USER_NOTFOUND } from '../error/errors';
import { Device } from "../generated/prisma";
import { verify_device } from "../utilities";

interface InitializeDeviceBody {
    device_key?: string
    device_pass?: string
}

interface ConnectControllableBody {
    controllable_name?: string
    device_key?: string
    device_pass?: string
}

export async function initialize_device(req: FastifyRequest<{ Body: InitializeDeviceBody }>, res: FastifyReply) {
    // Get the device key
    console.log("INITIALIZE");
    
    const { device_key: target_device_key, device_pass } = req.body;

    if(!target_device_key || !device_pass) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE.error_code);
    }

    // Verify the device key
    let device_data: Device | null;
    try {
        device_data = await prisma.device.findUnique({
            where: {
                device_key: target_device_key
            }
        });

        if(!device_data) {
            return res.code(404).send(ERROR_DEVICE_NOTFOUND.error_code);
        }
    }
    catch(error) {
        console.error(`There's an error when trying to get the device data. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR.error_code);
    }

    if(device_data.device_pass != device_pass) {
        return res.code(401).send(ERROR_DEVICE_WRONGPASS.error_code);
    }

    // Update the device status
    try {
        await prisma.device.update({
            where: {
                id: device_data.id
            },
            data: {
                status: 1
            }
        });
    }
    catch(error) {
        console.error(`There's an error when trying to update the device status. Error: ${error}`);
        return res.code(500).send(ERROR_UNKNOWN_ERROR.error_code);
    }

    return "Successfully Initialized"
}

export async function connect_to_controllable(req: FastifyRequest<{ Body: ConnectControllableBody }>, res: FastifyReply) {
    // Get the required body
    const { controllable_name, device_key, device_pass } = req.body;
    if(!controllable_name || !device_key || !device_pass) {
        return res.code(400).send(ERROR_REQUEST_BODY_NOTCOMPLETE.error_code);
    }


    // Verify device
    const device_data = await verify_device(device_key, device_pass);
    if(!device_data) {
        return res.code(401).send(ERROR_DEVICE_UNAUTHORIZED.error_code);
    }

    
    // Get the controllable data
    const controllable_data = await prisma.controllable.findUnique({
        where: {
            name_device_id: {
                device_id: device_data.id,
                name: controllable_name
            }
        }
    });

    if(!controllable_data) {
        return res.code(404).send(ERROR_CONTROLLABLE_NOTFOUND.error_code);
    }


    // Get the username and password
    const user_data = await prisma.user.findUnique({
        where: {
            email: device_data.user_email
        }
    });

    if(!user_data) {
        return res.code(404).send(ERROR_USER_NOTFOUND);
    }


    return controllable_data.topic_name+","+user_data.mqtt_user+","+user_data.mqtt_pass;
}