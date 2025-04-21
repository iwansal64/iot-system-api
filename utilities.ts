import { prisma } from "./database/database";

export function pick_random_from_array(array: any[] | string) {
    return array[(Math.floor(Math.random() * array.length))];
}


const TOKEN_KEY_LENGTH: number = 5; // Used to generate user verification token
const TOKEN_KEY_CHARACTERS: string = "abcdefghijklmnopqrstuvwxyz";

export function generate_verification_token() {
    let result: string = "";

    for(let i = 0; i < TOKEN_KEY_LENGTH; i++) {
        result += pick_random_from_array(TOKEN_KEY_CHARACTERS);
    }

    return result;
}

const DEVICE_KEY_LENGTH: number = 25; // Used to generate device key
const DEVICE_KEY_SEPERATOR: string = "-"; // Used to seperate per $ character
const DEVICE_KEY_SEPERATE_INTERVAL: number = 4; // Used to how many characters before putting the seperator
const DEVICE_KEY_CHARACTERS: string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

export function generate_device_key() {
    let result: string = "";

    for(let i = 0; i < DEVICE_KEY_LENGTH; i++) {
        result += pick_random_from_array(DEVICE_KEY_CHARACTERS);
        if(Math.min(i + 1, (DEVICE_KEY_LENGTH - 1)) % DEVICE_KEY_SEPERATE_INTERVAL == 0) {
            result += DEVICE_KEY_SEPERATOR;
        }
    }

    return result;
}


const DEVICE_PASS_LENGTH: number = 10;

export function generate_device_pass() {
    let result: string = "";

    for(let i = 0; i < DEVICE_PASS_LENGTH; i++) {
        result += pick_random_from_array(DEVICE_KEY_CHARACTERS);
    }

    return result;
}


const TOPIC_NAME_LENGTH: number = 50;

export function generate_topic_name() {
    let result: string = "";
    
    for(let i = 0; i < TOPIC_NAME_LENGTH; i++) {
        result += pick_random_from_array(DEVICE_KEY_CHARACTERS);
    }
    
    return result;
}


const MQTT_USER_PASS_LENGTH: number = 30;

export function generate_mqtt_user_and_pass() {
    let result: string = "";
    
    for(let i = 0; i < MQTT_USER_PASS_LENGTH; i++) {
        result += pick_random_from_array(DEVICE_KEY_CHARACTERS);
    }
    
    return result;
}

export async function verify_device(device_key: string, device_pass: string) {
    const device_data = await prisma.device.findUnique({
        where: {
            device_key: device_key
        }
    });

    if(!device_data || device_data.device_pass != device_pass) {
        return false
    }
    
    return device_data;
}