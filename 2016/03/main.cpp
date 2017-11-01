#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <algorithm>
#include <iterator>
#include <vector>

using namespace std;

bool is_triangle(vector<string> tokens) {
    vector<int> iv;
    iv.reserve(tokens.size());

    for (const auto &t : tokens) {
        iv.push_back(stoi(t));
    }
    sort(iv.begin(), iv.end());
    int shorts = iv[0] + iv[1];


    return shorts > iv[2];
}


int main()
{
    cout << "Triangles..." << endl;

    ifstream input;
    input.open("input", ios::in);

    if (! input.is_open()) {
        cout << "Could not open file" << endl;
        return 1;
    }
    string line;
    int triangles(0);
    int badtriangles(0);
    int rows_read(0);
    vector<string> col_1;
    vector<string> col_2;
    vector<string> col_3;
    col_1.reserve(3);
    col_2.reserve(3);
    col_3.reserve(3);

    while (getline(input, line)) {
        istringstream iss(line);
        vector<string> tokens{istream_iterator<string>{iss}, istream_iterator<string>{}};
        if (tokens.size() != 3) {
            cout << "bad line: " << line << endl;
            input.close();
            return 1;
        }
        rows_read++;
        col_1.push_back(tokens[0]);
        col_2.push_back(tokens[1]);
        col_3.push_back(tokens[2]);

        if (rows_read == 3) {
            is_triangle(col_1) ? triangles++ : badtriangles++;
            is_triangle(col_2) ? triangles++ : badtriangles++;
            is_triangle(col_3) ? triangles++ : badtriangles++;
            col_1.clear();
            col_2.clear();
            col_3.clear();
            rows_read = 0;
        }
    }
    if (rows_read != 0) {
        cout << "Line count is not divisible by 3: " << rows_read << endl;
    }

    input.close();

    cout << "Triangles: " << triangles << ", not triangles: " << badtriangles << endl;

    return 0;
}
