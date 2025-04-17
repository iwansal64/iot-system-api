import { fastify as Fastify } from "fastify";
import { initialize_device, request_key } from "./handlers/device";
import { apikey_validator, server_logger } from "./middlewares/security";
import dotenv from "dotenv";
import { connect_to_database, prisma } from "./database/database";
import { error_handler } from "./error/handler";

dotenv.config(); // .env File Initialization

const fastify = Fastify();

// Middleware
fastify.addHook("onRequest", apikey_validator);
fastify.addHook("onRequest", server_logger);
fastify.setErrorHandler(error_handler);

// Routes
fastify.post('/api/device/request_key', request_key);
fastify.post('/api/device/initialize', initialize_device);

// Run the server
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
        await fastify.listen({ port: Number.parseInt(process.env.API_PORT!) });
    } catch (err) {
        console.error("ERROR TRYING TO RUN THE SERVER.");
        console.error(err);
        process.exit(1);
    }
}

start()
