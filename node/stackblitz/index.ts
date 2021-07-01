import sdk from '@stackblitz/sdk'

window['embedGithubProject'] = () => {
  sdk.embedGithubProject(
    'myDiv',
    'nextjs-hwmqhp',
    {
      openFile: 'pages/index.tsx',
    }
  );
}
