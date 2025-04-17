import { PrismaClient } from '../generated/prisma/index';

const prisma = new PrismaClient();

async function connect_to_database() {
    await prisma.$connect();
    
    return true;
}

export { prisma, connect_to_database };
