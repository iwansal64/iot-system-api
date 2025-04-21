/*
  Warnings:

  - Added the required column `mqtt_pass` to the `User` table without a default value. This is not possible if the table is not empty.
  - Added the required column `mqtt_user` to the `User` table without a default value. This is not possible if the table is not empty.

*/
-- AlterTable
ALTER TABLE `user` ADD COLUMN `mqtt_pass` VARCHAR(191) NOT NULL,
    ADD COLUMN `mqtt_user` VARCHAR(191) NOT NULL;
