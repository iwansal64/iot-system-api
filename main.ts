import { fastify as Fastify } from "fastify";
import { connect_to_controllable, initialize_device } from "./handlers/device";
import { apikey_validator, server_logger, user_authentication } from "./middlewares/security";
import dotenv from "dotenv";
import { connect_to_database, prisma } from "./database/database";
import { error_handler } from "./error/handler";
import { log_in_user, verify_user, request_key, create_controllable, log_out_user } from "./handlers/application";
import { set_offline, set_online } from "./handlers/mqtt";
import fastifyCookie from "@fastify/cookie";
import fastifyJwt from "@fastify/jwt";

dotenv.config(); // .env File Initialization

const fastify = Fastify();

//SECTION Plugins
fastify.register(fastifyCookie, {
    secret: process.env.COOKIE_TOKEN, // for signed cookies (optional)
});
fastify.register(fastifyJwt, {
    secret: process.env.JWT_TOKEN!
});
//!SECTION


//SECTION Middleware
fastify.addHook("onRequest", apikey_validator);
fastify.addHook("onRequest", server_logger);
fastify.setErrorHandler(error_handler);
//!SECTION


//SECTION Routes
//ANCHOR Device Section
fastify.post('/api/device/initialize', initialize_device);
fastify.post('/api/device/connect_controllable', connect_to_controllable);

//ANCHOR Application Section
fastify.post('/api/app/login', log_in_user);
fastify.post('/api/app/logout', log_out_user);
fastify.post('/api/app/verify', verify_user);
fastify.post('/api/app/request_key', request_key);
fastify.post('/api/app/create_controllable', create_controllable);

//ANCHOR MQTT Section
fastify.post('/api/mqtt/set_online', set_online);
fastify.post('/api/mqtt/set_offline', set_offline);
//!SECTION


//SECTION Run the server
const start = async () => {
    try {
        await connect_to_database();
    }
    catch(err) {
        console.error("ERROR TRYING TO CONNECT TO DATABASE.");
        console.error(err);
        process.exit(1);
    }
    
    
    try {
	    console.log(`Listenning to port :${process.env.API_PORT}`);
        await fastify.listen({ port: Number.parseInt(process.env.API_PORT!), host: "0.0.0.0" });
    } catch (err) {
        console.error("ERROR TRYING TO RUN THE SERVER.");
        console.error(err);
        process.exit(1);
    }
}
//!SECTION


start()
