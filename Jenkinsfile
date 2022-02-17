pipeline {
    agent { label 'generic' }
    stages {
        stage('Release') {
            agent { dockerfile true }
            steps {
                sh 'echo hello'
            }
        }
    }
}