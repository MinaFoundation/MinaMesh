-- Drop the triggers
DROP TRIGGER if EXISTS trigger_add_to_user_commands_aggregated ON blocks_user_commands;

-- Drop the function
DROP FUNCTION if EXISTS update_user_commands_aggregated;

-- Drop indexes
DROP INDEX if EXISTS idx_user_commands_aggregated_hash;

-- Drop the table
DROP TABLE IF EXISTS user_commands_aggregated;
