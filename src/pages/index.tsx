import Image from "next/image";
import { Inter } from "next/font/google";
import { db } from "@/db/database";
import { useEffect, useState } from "react";
import * as schema from "@/db/schema";

const inter = Inter({ subsets: ["latin"] });

export default function Home() {
  const [name, setName] = useState("");
  const [users, setUsers] = useState<
    { id: number; name: string | null }[]
  >([]);

  async function addUser() {
    await db.insert(schema.users).values({ name });
    setName("");
    loadUsers();
  }

  useEffect(() => {
    async function init() {
      loadUsers();
      loadASingleUser();
    }
    init();
  }, []);

  const loadUsers = async () => {
    db.query.users
      .findMany()
      .execute()
      .then((results) => {
        console.log("🚀 ~ FindMany response from Drizzle:", results);
        setUsers(results);
      });
  };

  const loadASingleUser = async () => {
    db.query.users
      .findFirst()
      .execute()
      .then((result) => {
        console.log("🚀 ~ FindFirst response from Drizzle:", result);
      });
  };

  return (
    <div className={inter.className}>
      <h1 className="text-xl font-bold">Users</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          addUser();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          value={name}
          placeholder="Enter a name..."
          className="text-black"
        />
        <button type="submit">Add name to the db</button>
      </form>
      <ul>
        {users.map((user, index) => (
          <li key={index}>
            {user.id} - {user.name}
          </li>
        ))}
      </ul>
    </div>
  );
}
