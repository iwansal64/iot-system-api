/*
  Warnings:

  - You are about to drop the `topicauthorization` table. If the table is not empty, all the data it contains will be lost.
  - Added the required column `topic_name` to the `Controllable` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE `mqtttopic` DROP FOREIGN KEY `MQTTTopic_controllable_id_fkey`;

-- DropForeignKey
ALTER TABLE `topicauthorization` DROP FOREIGN KEY `TopicAuthorization_device_fkey`;

-- DropForeignKey
ALTER TABLE `topicauthorization` DROP FOREIGN KEY `TopicAuthorization_topic_fkey`;

-- DropIndex
DROP INDEX `MQTTTopic_controllable_id_key` ON `mqtttopic`;

-- AlterTable
ALTER TABLE `controllable` ADD COLUMN `topic_name` VARCHAR(191) NOT NULL;

-- DropTable
DROP TABLE `topicauthorization`;

-- AddForeignKey
ALTER TABLE `Controllable` ADD CONSTRAINT `Controllable_topic_name_fkey` FOREIGN KEY (`topic_name`) REFERENCES `MQTTTopic`(`topic_name`) ON DELETE RESTRICT ON UPDATE CASCADE;
