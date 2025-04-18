/*
  Warnings:

  - You are about to drop the `devicekey` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropForeignKey
ALTER TABLE `devicekey` DROP FOREIGN KEY `DeviceKey_device_key_fkey`;

-- DropTable
DROP TABLE `devicekey`;
