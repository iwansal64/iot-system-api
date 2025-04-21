/*
  Warnings:

  - You are about to drop the `mqtttopic` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE `controllable` DROP FOREIGN KEY `Controllable_topic_name_fkey`;

-- DropIndex
DROP INDEX `Controllable_topic_name_fkey` ON `controllable`;

-- DropTable
DROP TABLE `mqtttopic`;
