CREATE TABLE `dl_persons` (
	`id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name`	TEXT NOT NULL,
	`disabled`	INTEGER DEFAULT '0'
);

CREATE TABLE "dl_tokens" (
	`id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`person_id`	INTEGER NOT NULL,
	`token`	TEXT NOT NULL,
	`pin`	NUMERIC NOT NULL
);
