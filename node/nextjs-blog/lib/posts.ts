import fs from 'fs'
import path from 'path'
import matter from 'gray-matter'

const postsDirectory = path.join(process.cwd(), 'posts')

export type Post = {
  id? : string,
  title: string,
  date: string,
  tags: string[],
}

export function getSortedPostsData(): Post[] {
  // Get file names under /posts
  const fileNames = fs.readdirSync(postsDirectory)
  const allPostsData = fileNames.map(fileName => {
    // Remove ".md" from file name to get id
    const id = fileName.replace(/\.md$/, '')

    // Read markdown file as string
    const fullPath = path.join(postsDirectory, fileName)
    const fileContents = fs.readFileSync(fullPath, 'utf8')

    // Use gray-matter to parse the post metadata section
    const matterResult = matter(fileContents)
    const data = matterResult.data as Post;

    // Combine the data with the id
    return {
      id,
      ...data
    }
  })
  // Sort posts by date
  return allPostsData.sort((a, b) => {
    if (a.date < b.date) {
      return 1
    } else {
      return -1
    }
  })
}

export function getAllPostIds() {
  const fileNames = fs.readdirSync(postsDirectory)
  return fileNames.map(fileName => {
    return {
      params: {
        id: fileName.replace(/\.md$/, '')
      }
    }
  })
}

export async function getPostData(id): Promise<Post & {markdown: string}> {
  const fullPath = path.join(postsDirectory, `${id}.md`)
  const fileContents = fs.readFileSync(fullPath, 'utf8')

  // Use gray-matter to parse the post metadata section
  const matterResult = matter(fileContents)

  // Use remark to convert markdown into HTML string
  const markdown = matterResult.content;
  const data = matterResult.data as Post;
  // Combine the data with the id and contentHtml
  return {
    id,
    markdown,
    ...data
  }
}

export function getAllTagIds() {
  const fileNames = fs.readdirSync(postsDirectory);
  return fileNames.reduce((acc, fileName) => {
    const fileContents = fs.readFileSync(path.join(postsDirectory, fileName), 'utf8');
    const matterResult = matter(fileContents);
    return acc.concat(matterResult.data.tags);
  }, []).map(val => ({
    params: {
      id: val
    }
  }));
};


export function getTagData(id) {
  const fileNames = fs.readdirSync(postsDirectory);
  const posts = fileNames.map(fileName => {
    const fileContents = fs.readFileSync(path.join(postsDirectory, fileName), 'utf8');
    const matterResult = matter(fileContents);
    const data = matterResult.data as Post;
    return {
      id: fileName.replace(/\.md$/, ''),
      ...data
    };
  }).filter(({tags}) => tags.includes(id));
  return { id, posts };
};
