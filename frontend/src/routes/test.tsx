export default function test() {

    const handleSubmit = async (event: Event): Promise<void> => {
      try {
        const response = await fetch('http://localhost:8000/users/me', {
          method: 'GET',
          headers: {
            'Content-Type': 'application/json',
            // 'credentials': "include",
            'Access-Control-Allow-Origin': 'http://localhost:3000',
            'Access-Control-Allow-Credentials': 'true',
          },
          credentials: "include", // Don't forget to specify this if you need cookies
        });
    
        if (response.ok) {
          const data = await response.json();
          console.log('Response:', data);
          // Process the response data here
        } else {
          console.log('Error response:', response.status);
          const errorData = await response.json();
          console.log('Error data:', errorData);
          // Handle error response
        }
      } catch (error) {
        console.error('Error:', error);
        // Handle network or other errors
      }
    };
  
    return (
      <main class="text-center items-center mx-auto text-gray-700 p-4">
        <h1 class="text-4xl font-bold">Login</h1>
        <div class="p-20">
              <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
              type="submit" value="Submit" onClick={(event) => handleSubmit(event)}>
                  Sign In
              </button>
          <p class="text-center text-gray-500 text-xs">
              &copy;2020 Acme Corp. All rights reserved.
          </p>
          </div>
      </main>
    );
  }