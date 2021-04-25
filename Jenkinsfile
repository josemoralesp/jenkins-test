pipeline {
    agent any
    tools {
        nodejs '16.0.0'
	}
    options {
        timeout(time: 3, unit: 'MINUTES')
    }
    stages {
        stage('Install Dependencies') {
            steps {
                sh 'cd node-tests/sum-tests && npm i'
            }
        }
        stage('Run Test') {
            steps {
                sh 'cd node-tests/sum-tests && npm t'
            }
        }
        stage('Run Remote') {
            steps {
                build wait: false, job: 'Parameterized', parameters: [string(name: 'ROOT_ID', value: '$BUILD_ID')]
            }
        }
    }
}