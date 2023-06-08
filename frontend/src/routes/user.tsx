import { useParams } from "@solidjs/router";
import UserProfile from "~/components/UserProfile";

type UserProps = {
  username: string;
};

export default function User() {
  const params = useParams(); // 👈 Get the dynamic route parameters
  console.log(params);
  const username = params.username;
  return (
    <div>
      {/* <h1>{username}:</h1> */}
      <UserProfile username={username} />
    </div>
  );
}