/*
  Warnings:

  - A unique constraint covering the columns `[name,device_id]` on the table `Controllable` will be added. If there are existing duplicate values, this will fail.

*/
-- CreateIndex
CREATE UNIQUE INDEX `Controllable_name_device_id_key` ON `Controllable`(`name`, `device_id`);
