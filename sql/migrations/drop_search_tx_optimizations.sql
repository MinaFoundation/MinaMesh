-- Drop the triggers
DROP TRIGGER if EXISTS trigger_add_to_user_commands_aggregated ON blocks_user_commands;

DROP TRIGGER if EXISTS trigger_add_to_internal_commands_aggregated ON blocks_internal_commands;

-- Drop the function
DROP FUNCTION if EXISTS update_user_commands_aggregated;

DROP FUNCTION if EXISTS add_to_internal_commands_aggregated;

-- Drop indexes
DROP INDEX if EXISTS idx_user_commands_aggregated_hash;

DROP INDEX if EXISTS idx_internal_commands_aggregated_hash;

-- Drop the tables
DROP TABLE IF EXISTS user_commands_aggregated;

DROP TABLE IF EXISTS internal_commands_aggregated;

DROP TABLE IF EXISTS zkapp_commands_aggregated;
