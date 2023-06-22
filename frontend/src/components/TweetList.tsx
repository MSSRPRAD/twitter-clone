import TweetCard from "./TweetCard";

type TweetListProps = {
    tweets: Tweet[];
    profileUrl: string | null;
    quote_tweets: { [key: number]: Tweet};
};

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

export default function TweetList(props: TweetListProps) {
    console.log("tweetcard props - tweets");
    console.log(props.tweets);
    // console.log("tweetcard props - quote_tweets");
    // console.log(props.quote_tweets);
    return (
        <div>
            <ul>
                {props.tweets.map((tweet) => (
                    <li>
                        <TweetCard
                            tweet={tweet}
                            profileUrl={props.profileUrl}
                            quotedTweet={
                                tweet.quote_id && props.quote_tweets[tweet.quote_id]
                                  ? props.quote_tweets[tweet.quote_id]
                                  : null
                            }
                            quotedTweetProfileUrl={""}
                        />
                    </li>
                ))}
            </ul>
        </div>
    );
}