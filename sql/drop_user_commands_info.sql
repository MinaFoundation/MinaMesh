DROP TRIGGER if EXISTS refresh_user_command_info_on_blocks ON blocks;

DROP TRIGGER if EXISTS refresh_user_command_info_on_blocks_user_commands ON blocks_user_commands;

DROP TRIGGER if EXISTS refresh_user_command_info_on_user_commands ON user_commands;

DROP FUNCTION if EXISTS refresh_user_command_info;

DROP TABLE IF EXISTS user_command_info;
