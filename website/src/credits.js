
/**
 * Helper file for -> credits.md
 * Use Case: Fetches information about all the users who have forked this repository
 */

const https = require('https');

const options = {
    host: 'api.github.com',
    path: '/repos/romefrontend/rome/forks',
    headers: { 'User-Agent': 'romefrontend/rome'}
};

let forkList;

https.get(options , resp => {

    let data='';
    resp.on('data', chunk => {
        data += chunk;
    });

    resp.on('end', () => {

        // forst_list -> contains list of all the people who have forked this repository
        forkList = JSON.parse(data);

        if(forkList && forkList.length > 0) {
            forkList.forEach(function(fork)  {
                // fork.owner -> contains user information of a single person
                console.log(fork.owner.login)
            })
        } else {
            forkList = [];
        }
    })

}).on("error", err => {
    console.log(`Error ${err.message}`)
});

