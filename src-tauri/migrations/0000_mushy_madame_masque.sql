CREATE TABLE `users` (
	`age` integer DEFAULT 18,
	`id` integer PRIMARY KEY NOT NULL,
	`name` text
);
--> statement-breakpoint
CREATE UNIQUE INDEX `users_id_unique` ON `users` (`id`);