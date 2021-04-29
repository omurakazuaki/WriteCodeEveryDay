import React from 'react'
import { Link } from 'gatsby'
import SEO from "../components/seo"
import Layout from '../components/layout'
import ReactMarkdown from 'react-markdown'
const PostTemplate = ({ data, pageContext }) => {
  return (
    <Layout>
      <SEO title={pageContext.data.title} />
      <h1>{pageContext.data.title}</h1>
      <ReactMarkdown>
        {pageContext.data.content}
      </ReactMarkdown>
      <p>
        <Link to="/">top</Link>
      </p>
    </Layout>
  )
}

export default PostTemplate;
