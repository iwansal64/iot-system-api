import { Prisma } from "../generated/prisma";

export function is_duplicate(error: any): boolean {
    return (error instanceof Prisma.PrismaClientKnownRequestError);
}