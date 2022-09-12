# arcast

A tool to let you archive episodes of a podcast.

In the future, I hope to better document the configuration file format, but for now, take this example of a configuration file for the [Accidental Tech Podcast](https://atp.fm):

```json
{
	"title": "Accidental Tech Podcast",
	"url": "https://atp.fm/episodes?format=rss",
	"compareBasedOnFilename": true
}
```

If you store that as, say, `atp_config.json`, then you can run

```sh
arcast --pretend --config-file-path atp_config.json --destination .
```

and arcast will [pretend to] download every episode of ATP to your current directory.
