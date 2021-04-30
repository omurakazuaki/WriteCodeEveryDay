import React from 'react'
import { Link, graphql } from 'gatsby'
import SEO from "../components/seo"
import Layout from '../components/layout'
import ReactMarkdown from 'react-markdown'

const PostTemplate = ({ data, _ }) => {
  return (
    <Layout>
      <SEO title={data.strapiPost.title} />
      {
        data.strapiPost.tags.map(({name})=> {
          return <span style={{marginRight: '16px'}}>
            <Link to={`/tag/${name}`}>{name}</Link>
          </span>
        })
      }
      <h1>{data.strapiPost.title}</h1>
      <ReactMarkdown>
        {data.strapiPost.content}
      </ReactMarkdown>
      <p>
        <Link to="/">top</Link>
      </p>
    </Layout>
  )
}

export const pageQuery =
  graphql`
    query($slug: String) {
      strapiPost(slug: {eq: $slug}) {
        title
        content
        tags{name}
        created_at
      }
  }`;

export default PostTemplate;
