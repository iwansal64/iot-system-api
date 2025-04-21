/*
  Warnings:

  - You are about to drop the column `controllable_id` on the `mqtttopic` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE `mqtttopic` DROP COLUMN `controllable_id`;
