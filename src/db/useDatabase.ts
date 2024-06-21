import { useEffect, useState } from "react";
import Database from "tauri-plugin-sql-api";

const useDatabase = () => {
  const [database, setDatabase] = useState<Database>();
  useEffect(() => {
    const loadDatabase = async () => {
      const db = await Database.load("sqlite:test.db");
      setDatabase(db);
    };
    loadDatabase();
  }, []);
  return database;
};

export default useDatabase;
