import { Setter, createSignal } from 'solid-js';

interface CreateTweetCardProps {
  parent_id: number | null;
  quote_id: number | null;
  addTweet: (tweet: Tweet) => void;
}

type Tweet = {
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

interface CreateTweetModelResponse {
  parent_id: number | null;
  content: string;
  quote_id: number | null;
}

export default function CreateTweetCard(props: CreateTweetCardProps) {
  const [content, setContent] = createSignal('');

  const handleSubmit = async () => {
    const tweetData: CreateTweetModelResponse = {
      parent_id: props.parent_id,
      content: content(),
      quote_id: props.quote_id,
    };
    console.log("creating:");
    console.log(JSON.stringify(tweetData));
    const response = await fetch("http://localhost:8000/tweets/me", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(tweetData),
        credentials: "include", // Don't forget to specify this if you need cookies
      });
    console.log("create tweet's response:");
    const new_tweet = await response.json();
    console.log(new_tweet);
    props.addTweet(new_tweet);
  };

  return (
    <div>
      <div>
        <textarea
          class="w-full border-stone-100 border-2 p-5 m-5 bg-stone-700 text-gray-400 font-large text-lg w-full"
          rows={3}
          cols={50}
          placeholder="What's happening?"
          value={content()}
          onInput={(e) => setContent(e.target.value)}
        ></textarea>
        <br />
        <button
          class="w-20 h-10 ms-10 mt-0 mb-5 flex justify-center bg-blue-400 max-h-max whitespace-nowrap focus:outline-none focus:ring rounded border bg-transparent border-blue-500 text-blue-500 hover:border-blue-800 hover:border-blue-800 flex items-center hover:shadow-lg font-bold py-2 px-4 rounded-full mr-0 ml-auto"
          onClick={handleSubmit}
        >
          Tweet!
        </button>
      </div>
    </div>
  );
}