import { MDXProvider } from "@mdx-js/react"

const MyApp = ({ Component, pageProps }) => {
  return (
    <MDXProvider>
      <Component { ...pageProps } />
    </MDXProvider>
  )
}

export default MyApp
