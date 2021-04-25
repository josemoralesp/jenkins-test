pipeline {
    agent any
    tools {
        nodejs 'node-16.0.0'
	}
    options {
        timeout(time: 2, unit: 'MINUTES')
    }
    stages {
        stage('Install dependencies') {
            steps {
                sh 'cd node-tests/sum-tests && npm i'
            }
        }
        state('Run test') {
            steps {
                sh 'cd node-tests/sum-tests && npm t'
            }
        }
    }
}
