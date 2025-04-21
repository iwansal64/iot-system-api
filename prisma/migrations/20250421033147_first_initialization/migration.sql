/*
  Warnings:

  - You are about to drop the column `token` on the `device` table. All the data in the column will be lost.
  - You are about to drop the column `workspace_id` on the `device` table. All the data in the column will be lost.
  - You are about to drop the `workspace` table. If the table is not empty, all the data it contains will be lost.
  - A unique constraint covering the columns `[device_key]` on the table `Device` will be added. If there are existing duplicate values, this will fail.
  - A unique constraint covering the columns `[device_pass]` on the table `Device` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `device_key` to the `Device` table without a default value. This is not possible if the table is not empty.
  - Added the required column `device_pass` to the `Device` table without a default value. This is not possible if the table is not empty.
  - Added the required column `user_email` to the `Device` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE `controllable` DROP FOREIGN KEY `Controllable_device_id_fkey`;

-- DropForeignKey
ALTER TABLE `device` DROP FOREIGN KEY `Device_workspace_id_fkey`;

-- DropForeignKey
ALTER TABLE `workspace` DROP FOREIGN KEY `Workspace_user_id_fkey`;

-- DropIndex
DROP INDEX `Controllable_device_id_fkey` ON `controllable`;

-- DropIndex
DROP INDEX `Device_token_key` ON `device`;

-- DropIndex
DROP INDEX `Device_workspace_id_fkey` ON `device`;

-- AlterTable
ALTER TABLE `device` DROP COLUMN `token`,
    DROP COLUMN `workspace_id`,
    ADD COLUMN `device_key` VARCHAR(191) NOT NULL,
    ADD COLUMN `device_pass` VARCHAR(191) NOT NULL,
    ADD COLUMN `user_email` VARCHAR(191) NOT NULL;

-- DropTable
DROP TABLE `workspace`;

-- CreateTable
CREATE TABLE `MQTTTopic` (
    `id` VARCHAR(191) NOT NULL,
    `topic_name` VARCHAR(191) NOT NULL,
    `created_at` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    `last_update` DATETIME(3) NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    `controllable_id` VARCHAR(191) NOT NULL,

    UNIQUE INDEX `MQTTTopic_topic_name_key`(`topic_name`),
    UNIQUE INDEX `MQTTTopic_controllable_id_key`(`controllable_id`),
    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `TopicAuthorization` (
    `id` VARCHAR(191) NOT NULL,
    `topic` VARCHAR(191) NOT NULL,
    `device` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateIndex
CREATE UNIQUE INDEX `Device_device_key_key` ON `Device`(`device_key`);

-- CreateIndex
CREATE UNIQUE INDEX `Device_device_pass_key` ON `Device`(`device_pass`);

-- AddForeignKey
ALTER TABLE `Device` ADD CONSTRAINT `Device_user_email_fkey` FOREIGN KEY (`user_email`) REFERENCES `User`(`email`) ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Controllable` ADD CONSTRAINT `Controllable_device_id_fkey` FOREIGN KEY (`device_id`) REFERENCES `Device`(`id`) ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `MQTTTopic` ADD CONSTRAINT `MQTTTopic_controllable_id_fkey` FOREIGN KEY (`controllable_id`) REFERENCES `Controllable`(`id`) ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `TopicAuthorization` ADD CONSTRAINT `TopicAuthorization_topic_fkey` FOREIGN KEY (`topic`) REFERENCES `MQTTTopic`(`id`) ON DELETE CASCADE ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `TopicAuthorization` ADD CONSTRAINT `TopicAuthorization_device_fkey` FOREIGN KEY (`device`) REFERENCES `Device`(`id`) ON DELETE CASCADE ON UPDATE CASCADE;
