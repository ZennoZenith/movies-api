const imdb_tsv_urls = [
    https://datasets.imdbws.com/name.basics.tsv.gz
    https://datasets.imdbws.com/title.akas.tsv.gz
    https://datasets.imdbws.com/title.basics.tsv.gz
    https://datasets.imdbws.com/title.crew.tsv.gz
    https://datasets.imdbws.com/title.episode.tsv.gz
    https://datasets.imdbws.com/title.principals.tsv.gz
    https://datasets.imdbws.com/title.ratings.tsv.gz
]

def check_installed_packages [] {
    # aim
    true 
}


def main [] {
    if check_installed_packages == false {
        return 1
    }

    if ("movies-api" != ( $env.PWD | path basename ) ) {
        print "Change to movies-api directory"
        return 1
    }

    ^mkdir -p ./temp/imdb-tsv

    cd ./temp/imdb-tsv

    $imdb_tsv_urls | each {
        let url = $in
        try {
            aim $url "+"
        } catch {
            print $"($url) errored"
        }
        print $"($url) downloaded"
    }

}
