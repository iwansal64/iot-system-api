/*
  Warnings:

  - Added the required column `token` to the `Device` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `device` ADD COLUMN `token` VARCHAR(191) NOT NULL;
