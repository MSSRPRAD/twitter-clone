import { useParams } from "@solidjs/router";
import UserProfile from "~/components/UserProfile";
import { createSignal, createEffect } from 'solid-js';
import TweetList from "~/components/TweetList";
import CreateTweetCard from "~/components/CreateTweetCard";
import Tweet from "~/types/Tweet";

type Data = {
  tweets: TweetDetails[];
}

type UserTweetsResponse = {
  status: String;
  results: number;
  quoted_tweets: { [key: number]: Tweet };
  tweets: Tweet[];
  profile_pics: { [key: number]: String };
}

type TweetDetails = {
  content: string;
  created_at: string;
  likes: number;
  parent_id: number | null;
  quote_id: number | null;
  quotes: number;
  replies: number;
  retweets: number;
  tweet_id: number;
  username: string;
  views: number;
};


export default function Timeline() {
  const params = useParams(); // 👈 Get the dynamic route parameters
  console.log(params);
  const username = params.username;

 const [tweets, setTweets] = createSignal([] as  Tweet[]);
  const [quote_tweets, setQuoteTweets] = createSignal({} as { [key: number]: Tweet });
  const [profileUrl, setProfileUrl] = createSignal("");
  const [profilePics, setProfilePics] = createSignal( {} as { [key: number]: String } );

  // const test = profileUrl();
  const addUrl = (url: string) => {
    setProfileUrl(url);
  }

  createEffect(async () => {
    const _resp = await fetch_tweets(username);
  });

  const setStuff = (tweets: Tweet[], quoted_tweets: {[key: number]: Tweet}, profile_pics: { [key: number]: String }) => {
    setTweets(tweets);
    setQuoteTweets(quoted_tweets);
    setProfilePics(profile_pics);
  }

  async function fetch_tweets(username: string): Promise<UserTweetsResponse | null> {
  try {
    const response = await fetch('http://localhost:8000/twitter/timeline/me', {
      method: 'GET',
      credentials: "include",
    })
    if (response.ok) {
      const data = await response.json();
      console.log("data");
      console.log(data);
      setStuff(data.tweets, data.quoted_tweets, data.profile_urls);
      return data as UserTweetsResponse;
    } else {
      console.error('Error fetching tweets:', response.status);
      return null;
    }
  } catch (error) {
    console.error('Error fetching tweets:', error);
    return null;
  }
}
// Argument of type '(prev: Tweet[]) => { tweets: Tweet[]; }' is not assignable to parameter of type '(prev: Tweet[]) => Tweet[]'.
  const addTweet = (tweet: Tweet) => {
    setTweets(prev => {
      return  [tweet, ...prev] as Tweet[];
    });
  }

  return (
    <div class="bg-gray-800 w-full">
      <div class="flex items-center">
        <CreateTweetCard quote_id={null} parent_id={null} addTweet = {addTweet} />
      </div>
      <ul class="list-none">
        <TweetList quote_tweets={quote_tweets()} tweets={tweets() as Tweet[]} profileUrl = {null} profile_pics = { profilePics() }/>
      </ul>
    </div>
  );
}
