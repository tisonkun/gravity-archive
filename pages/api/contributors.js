import { Octokit } from "octokit";

export default async function handle(req, res) {
    const octokit = new Octokit();

    let contributors = [];
    for (let page = 1; ; page++) {
        const response = await octokit.rest.repos.listContributors({
            owner: 'pingcap',
            repo: 'tidb',
            per_page: 100,
            page: page,
        })
        if (response.status != 200) {
            res.status(response.status);
            return;
        }
        if (!response.data.length) {
            break;
        }
        contributors = contributors.concat(response.data.map(c => {
            return {
                login: c.login,
                html_url: c.html_url,
            };
        }));
        console.log(contributors);
    }
    res.status(200).json(contributors);
}
