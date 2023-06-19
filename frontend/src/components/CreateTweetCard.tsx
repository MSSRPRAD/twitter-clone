export default function CreateTweetCard() {
    return (
    <div>
        <div>
            <textarea class="w-full border-stone-100 border-2 p-5 m-5 bg-stone-700 text-gray-400 font-large text-lg w-full" rows="3" cols="50" placeholder="What's happening?"></textarea>
            <br />
            <button class="w-20 h-10 ms-10 mt-0 mb-5 flex justify-center bg-blue-400 max-h-max whitespace-nowrap focus:outline-none  focus:ring rounded border bg-transparent border-blue-500 text-blue-500 hover:border-blue-800 hover:border-blue-800 flex items-center hover:shadow-lg font-bold py-2 px-4 rounded-full mr-0 ml-auto">
                Tweet!
            </button>
        </div>
    </div>
    );
}