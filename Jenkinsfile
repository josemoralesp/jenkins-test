pipeline {
    agent any
    tools {
        nodejs '16.0.0'
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
        stage('Run test') {
            steps {
                sh 'cd node-tests/sum-tests && npm t'
            }
        }
    }
}
