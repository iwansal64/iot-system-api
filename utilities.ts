const TOKEN_KEY_LENGTH: number = 5; // Used to generate device key
const TOKEN_KEY_CHARACTERS: string = "abcdefghijklmnopqrstuvwxyz";

export function pick_random_from_array(array: any[] | string) {
    return array[(Math.floor(Math.random() * array.length))];
}

export function generate_verification_token() {
    let result: string = "";
    for(let i = 0; i < TOKEN_KEY_LENGTH; i++) {
        result += pick_random_from_array(TOKEN_KEY_CHARACTERS);
    }
    return result;
}