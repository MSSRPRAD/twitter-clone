import React from 'react';

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

interface UserListProps {
  users: UserModelResponse[];
}

const UserList: React.FC<UserListProps> = ({ users }) => {
  return (
    <div className="max-w-md mx-auto mt-8">
      <h1 className="text-2xl font-bold mb-4">User List</h1>
      <ul className="space-y-4">
        {users.map((user) => (
          <li key={user.username} className="bg-white shadow-md p-4 rounded-lg">
          <p className="text-gray-600 font-semibold">Username: {user.username}</p>
          <p className="text-gray-800">Name: {user.name}</p>
            <p className="text-gray-800">Role: {user.role_id}</p>
            <p className="text-gray-600">Email: {user.email}</p>
            <p className="text-gray-600">Date of Birth: {user.dob}</p>
          </li>
        ))}
      </ul>
    </div>
  );
};

export default UserList;
