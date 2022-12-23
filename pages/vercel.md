- github action to deploy to vercel:

  ```
        - name: Deploy to vercel
          env:
            VERCEL_ORG_ID: ${{secrets.VERCEL_ORG_ID}}
            VERCEL_PROJECT_ID: ${{secrets.VERCEL_PROJECT_ID}}
          run: |
            npm i -g vercel
            vercel --token=${{ secrets.VERCEL_TOKEN }} --prod --scope <username> <dir>
  ```
