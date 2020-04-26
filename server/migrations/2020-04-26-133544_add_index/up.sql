-- Your SQL goes here
CREATE UNIQUE INDEX `tweets_to_tags_index_2` ON `tweets_to_tags` (`tweets_id`, `tags_id`);