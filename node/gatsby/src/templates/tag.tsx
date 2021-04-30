import React from 'react'
import { Link, graphql } from 'gatsby'
import SEO from "../components/seo"
import Layout from '../components/layout'

const TagTemplate = ({ data, _ }) => {
  return (
    <Layout>
      <SEO title={data.strapiTag.name} />
      <h1>{data.strapiTag.name} Tag</h1>
      <ul>
        {
          data.strapiTag.posts.map(post => {
            return <li><Link to={`/post/${post.slug}`}>{post.title}</Link> {post.created_at}</li>;
          })
        }
      </ul>
      <p>
        <Link to="/">top</Link>
      </p>
    </Layout>
  )
}

export const pageQuery =
  graphql`
    query($name: String) {
      strapiTag(name: {eq: $name}) {
        name
        posts {
          slug
          title
          created_at
        }
      }
  }`;

export default TagTemplate;
