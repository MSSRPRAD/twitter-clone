import "./App.css";
import { useState, useEffect } from "react";
import UserList from "./components/UserList";
import axios from "axios";

interface RegisterUserSchema {
  role_id: number;
  username: string;
  name: string;
  email: string;
  password: string;
  dob: string;
}

interface UserModelResponse {
  role_id: number;
  user_id: number;
  name: string;
  username: string;
  email: string;
  created_at: string;
  dob: string;
  profile_id: string;
  password: string;
}

function App() {
  const [users, setUsers] = useState<UserModelResponse[]>([]);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await axios.get("http://127.0.0.1:8000/users/all");
        const data = response.data;
        setUsers(data.users);
      } catch (error) {
        console.error("Error fetching user data:", error);
      }
    };

    fetchData();
  }, []);
  console.log("users: " + users);

  return (
    <div>
      <div id="userlist">
        <UserList users={users} />
      </div>
    </div>
  );
}

export default App;
