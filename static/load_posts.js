async function fetchPosts() {
    try {
        const response = await fetch('/api/get_posts');
        if (!response.ok) {
            throw new Error('Network response was not ok');
        }
        const posts = await response.json();
        displayPosts(posts);
    } catch (error) {
        console.error('Error fetching posts:', error);
    }
}

function displayPosts(posts) {
    const container = document.getElementById('posts-container');
    container.innerHTML = ''; // Clear previous posts

    posts.forEach(post => {
        const postElement = document.createElement('div');
        postElement.className = 'post';

        const avatar = document.createElement('img');
        avatar.src = post.avatar_path;
        avatar.alt = `${post.username}'s avatar`;
        avatar.className = 'avatar';

        const username = document.createElement('span');
        username.className = 'username';
        username.textContent = post.username;

        const publishDate = document.createElement('span');
        publishDate.className = 'publish-date';
        publishDate.textContent = ` • ${new Date(post.publish_date).toLocaleDateString()} ${new Date(post.publish_date).toLocaleTimeString()}`;

        const body = document.createElement('p');
        body.className = 'body';
        body.textContent = post.body;

        postElement.appendChild(avatar);
        postElement.appendChild(username);
        postElement.appendChild(publishDate);
        postElement.appendChild(body);

        // If there is an image, add it
        if (post.image_path) {
            const image = document.createElement('img');
            image.src = post.image_path;
            image.alt = 'Post image';
            image.className = 'image';
            postElement.appendChild(image);
        }

        container.appendChild(postElement);
    });
}