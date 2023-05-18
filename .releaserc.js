module.exports = {
  release: {
    plugins: [
      "@semantic-release/commit-analyzer",
      "@semantic-release/release-notes-generator",
      "@semantic-release/changelog",
      "@semantic-release/github",
      "@semantic-release/git"
    ],
    branch: "master",
    preset: "conventionalcommits"
  },
}
